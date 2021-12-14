addpath(".");

close all
clear
clc
format long g

root_path = 'D:\Research\NFV_PlacementModel_Journal\';
% root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, 'data', 'AltModels');
out_folder = fullfile(root_path, 'processed', 'pcp');

sizes = [12, 16, 20, 24, 28, 32];
runs = [10, 10, 10, 10, 10, 10, 10, 10];

for i = 1:length(sizes)
    size = num2str(sizes(i));
    run = num2str(runs(i));
    
    alg_folder = fullfile(src_folder, size, run);
    
    hr_path = fullfile(alg_folder, 'hr', 'IBEA');
    qm_path = fullfile(alg_folder, 'qm', 'IBEA');
    
    hr_file = dir(fullfile(hr_path, '*_12000.objs'));
    qm_file = dir(fullfile(qm_path, '*_12000.objs'));
    
    % Read in file and get non-dominated solutions
    simple_objs = get_objs(hr_file);
    ca_objs = get_objs(qm_file);
    
    % Write out data
    a_out = fullfile(out_folder, size);
    make_if_not_exists(a_out);
    
    writematrix(simple_objs, fullfile(a_out, 'hr.csv'));
    writematrix(ca_objs, fullfile(a_out, 'qm.csv'));
    writematrix([["la", "pl", "en"]; ca_objs; simple_objs], fullfile(a_out, 'both.csv'), 'Delimiter', ' ');
    
%     sgtitle(qm_file.name);
%     %     parallelplot([ca_objs(:, 2), ca_objs(:, 1), ca_objs(:, 3)]);
%     plot3(ca_objs(:, 2), ca_objs(:, 1), ca_objs(:, 3), 'o');
end

function objectives = get_objs(file)

file_path = fullfile(file.folder, file.name);

% Manually read CSV to handle 'Infeasible' values
fid = fopen(file_path);
lines = {};
tline = fgetl(fid);

objectives = [];
row = 1;

while ischar(tline)
    if contains(tline, 'Infeasible')
        tline = fgetl(fid);
        continue
    end
    
    s = split(tline, ',');
    
    objectives(row,1) = str2num(s{1});
    objectives(row,2) = str2num(s{2});
    objectives(row,3) = str2num(s{3});
    
    tline = fgetl(fid);
    
    row = row + 1;
end
fclose(fid);

objectives = filter_NDS(objectives, objectives);
objectives = unique(objectives, 'rows');

end

function make_if_not_exists(dest_folder)
if ~exist(dest_folder, 'dir')
    mkdir(dest_folder);
end
end