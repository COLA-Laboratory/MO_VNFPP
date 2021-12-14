addpath(".");

close all
clear
clc
format long g

%% Parameters
num_objectives = 3;

runs = 30;

% root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "Aggregate", "Comparison");
out_folder = fullfile(root_path, "processed", "means");

if exist(out_folder, 'dir')
    rmdir(out_folder, 's')
end

alg_paths = [fullfile("IBEA", "ca_operators"),...
    fullfile("IBEA", "simple_model"),...
    fullfile("IBEA", "standard_ga"),...
    fullfile("RoundRobin")];

sizes = dir(src_folder);
sizes = natsortfiles({sizes.name});
sizes = sizes(3:end); % Remove . and ..

for size = sizes
    for alg_path = alg_paths
        in_file = fullfile(src_folder, size, alg_path, "aggregate.csv");
        
        % Only some runs of Standard GA exist
        if isfile(in_file)
            in_file = csvread(in_file); % Skip header
            hv = in_file(end, 2);
        else
            hv = '0';
        end
        
        out_part_folder = fullfile(out_folder, alg_path);
        out_file = fullfile(out_part_folder, 'final_hv.csv');
        
        if ~exist(out_part_folder, 'dir')
            mkdir(out_part_folder)
        end
        
        fid = fopen(out_file, 'a+');
        fprintf(fid, '%d,%f\n', str2double(size), hv);
        fclose(fid);
    end
end