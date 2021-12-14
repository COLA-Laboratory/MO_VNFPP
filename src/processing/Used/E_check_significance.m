addpath('.');

close all
clear
clc
format long g

%% Parameters
num_objectives = 3;

runs = 30;

root_path = '/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal/results/MOEA/';
% root_path = 'D:\Research\NFV_PlacementModel_Journal';

sizes = ["12", "16", "20", "24", "28", "32"];
algs = ["NSGAII", "IBEA", "MOEAD"];

for i = 1:length(sizes)
    for j = 1:length(algs)
        size = sizes(i);
        alg = algs(j);
        
        folder = fullfile(root_path, size, alg);
        
        file_search = fullfile(folder, '*', 'HV.out');
        hv_files = dir(file_search);
        
        agg_hv = [];
        
        for k = 1 : length(hv_files)
            hv_file = hv_files(k);
            file = fullfile(hv_file.folder, hv_file.name);
            hvs = csvread(file);
            
            agg_hv = [agg_hv, hvs(end, 2)];
        end
        
        out(:, j) = agg_hv;
    end
    
    fprintf('%s\n', size);
    for j = 1:length(algs)
        for k = 1:length(algs)
            if j == k
                continue
            end
            
            [sig_test, h] = ranksum(out(:, j), out(:, k), 'tail', 'right');
            fprintf('(%s,%s): %f, %i \n', algs(j), algs(k), sig_test, h);
        end
    end
    fprintf('\n', size);
end